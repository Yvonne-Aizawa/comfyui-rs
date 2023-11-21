use handlebars::Handlebars;
use rand::Rng;
use std::collections::{BTreeMap, HashMap};
use std::{fs::File, io::Write};
use tungstenite::connect;
use url::Url;
use uuid::Uuid;
mod structs;
use structs::{ComfyUI, GenerationResponse, HistoryData};
mod tests;

impl ComfyUI {
    pub async fn get_history(
        self,
        promt_id: String,
    ) -> Result<HashMap<String, HistoryData>, serde_json::Error> {
        let client = reqwest::Client::new();

        let res = client
            .get(format!("{}/history/{}", self.format_url(), promt_id))
            .send()
            .await
            .unwrap();
        let data: Result<HashMap<String, HistoryData>, serde_json::Error> =
            serde_json::from_str(&res.text().await.unwrap());
        if let Ok(obj) = data {
            return Ok(obj);
        } else {
            return Err(data.err().unwrap());
        }
    }
    pub async fn get_image(
        self,
        filename: &String,
        subfolder: &String,
        folder_type: &String,
    ) -> String {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "{}/view?filename={}&subfolder={}&type={}",
                self.format_url(),
                filename,
                subfolder,
                folder_type
            ))
            .send()
            .await
            .unwrap();
        //write to tmp file
        let mut file = File::create("test.jpg").unwrap();
        file.write_all(&res.bytes().await.unwrap());
        return "hi".to_string();
        // return res.bytes().await.unwrap();
    }
    pub async fn send(self, json: String) -> Result<String, String> {
        let client = reqwest::Client::new();

        let res = client
            .post("http://localhost:8188/prompt")
            .body(json)
            .send()
            .await;
        if let Ok(resp) = res {
            let resp: Result<GenerationResponse, serde_json::Error> =
                serde_json::from_str(&resp.text().await.unwrap());
            if let Ok(json) = resp {
                //open websocket
                let id = json.prompt_id;

                let (mut socket, response) = connect(
                    Url::parse(&format!(
                        "ws://localhost:8188/ws?clientId={}",
                        self.client_id
                    ))
                    .unwrap(),
                )
                .expect("Can't connect");
                loop {
                    //imagine it failing and waiting forever. that is what this code does pls fix that
                    let msg = socket.read().unwrap().to_string();
                    let success = format!("{{\"node\": null, \"prompt_id\": \"{}\"}}", &id);
                    if msg.contains(&success) {
                        let history = self.clone().get_history(id.clone()).await;
                        if let Ok(h) = history {
                            let outputs = &h.get(&id).unwrap().outputs;
                            let image = &outputs.into_iter().next().unwrap().1.images.first();
                            if let Some(i) = image {
                                self.clone()
                                    .get_image(&i.filename, &i.subfolder, &i.image_type)
                                    .await;
                            }
                        }
                        return Ok("generation succeded".to_string());
                    }
                    let fail = format!(
                        "{{\"type\": \"execution_error\", \"data\": {{\"prompt_id\": \"{}\"",
                        id
                    );
                    if msg.contains(&fail) {
                        return Err("generation failed".to_string());
                    }
                }
            } else {
                return Err("Server returned something we did not understand".to_string());
            }
        } else {
            return Err("could not connect to server".to_string());
        }
    }
    pub async fn simple_promt(
        self,
        positive: String,
        negative: String,
        model: String,
        seed: Option<u32>,
    ) -> Result<String, String> {
        let mut handlebars = Handlebars::new();
        let mut data = BTreeMap::new();

        let contents = include_str!("templates/simple.json.hbs");
        if let Some(s) = seed {
            data.insert("seed".to_string(), s.to_string());
        } else {
            let mut rng = rand::thread_rng();
            // TODO: generate a random number here
            data.insert("seed".to_string(), rng.gen::<u32>().to_string());
        }
        if handlebars.register_template_string("t1", contents).is_ok() {
            data.insert("model".to_string(), model);
            data.insert("positive".to_string(), positive);
            data.insert("negative".to_string(), negative);
            data.insert("client_id".to_string(), self.client_id.to_string());
            let rendered = handlebars.render("t1", &data).unwrap();
            let response = self.send(rendered).await;
            return response;
        }
        Err("Failed to render json".to_string())
    }

    pub async fn lora_promt(
        self,
        positive: String,
        negative: String,
        lora_1: String,
        lora_2: String,
        lora_3: String,
        model: String,
        seed: Option<u32>,
    ) -> Result<String, String> {
        let mut handlebars = Handlebars::new();
        let mut data = BTreeMap::new();

        let contents = include_str!("templates/lora.json.hbs");
        if let Some(s) = seed {
            data.insert("seed".to_string(), s.to_string());
        } else {
            let mut rng = rand::thread_rng();
            // TODO: generate a random number here
            data.insert("seed".to_string(), rng.gen::<u32>().to_string());
        }
        if handlebars.register_template_string("t1", contents).is_ok() {
            data.insert("model".to_string(), model);
            data.insert("positive".to_string(), positive);
            data.insert("negative".to_string(), negative);
            data.insert("lora_name_1".to_string(), lora_1);
            data.insert("lora_name_2".to_string(), lora_2);
            data.insert("lora_name_3".to_string(), lora_3);
            data.insert("client_id".to_string(), self.client_id.to_string());
            let rendered = handlebars.render("t1", &data).unwrap();
            let response = self.send(rendered).await;
            return response;
        }
        Err("Failed to render json".to_string())
    }
}
impl Default for ComfyUI {
    fn default() -> Self {
        ComfyUI {
            client_id: Uuid::new_v4(),
            url: "http://localhost".to_string(),
            port: 8188,
        }
    }
}
