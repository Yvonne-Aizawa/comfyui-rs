use crate::structs::ComfyUI;
#[tokio::test]
async fn test_simple_promt() {
    let comfyUI = ComfyUI::default();
    comfyUI
        .simple_promt(
            "a dog sitting on a bench".to_string(),
            "text, watermark embedding:EasyNegative, embedding:FastNegativeV2, embedding:ng_deepnegative_v1_75t, embedding:verybadimagenegative_v1.3,".to_string(),
            "dreamshaper_8.safetensors".to_string(),
            None,
        )
        .await;
}

#[tokio::test]
async fn test_lora_promt() {
    let comfyUI = ComfyUI::default();
    comfyUI.lora_promt("a dog sitting on a bench".to_string(),
    "text, watermark embedding:EasyNegative, embedding:FastNegativeV2, embedding:ng_deepnegative_v1_75t, embedding:verybadimagenegative_v1.3,".to_string(),
    "None".to_string(),
    "None".to_string(),
    "None".to_string(),
    "dreamshaper_8.safetensors".to_string(), 
     None
    ).await;
}
