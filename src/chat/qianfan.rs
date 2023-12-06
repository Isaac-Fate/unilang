// use anyhow::{Result, anyhow};
// use crate::{
//     chat::{
//         ChatModel, 
//         ChatModelName, 
//         ChatMessage,
//         ChatRole,
//         ChatResponse,
//         ChatTokenUsage,
//     },
//     qianfan::{
//         self,
//         chat::{
//             QianFanChatRequestBody, 
//             QianFanChatCompletion, 
//             QianFanChatMessage,
//             QianFanChatRole,
//         }
//     },
// };

// pub async fn get_complete_chat_response(
//     model: &ChatModel,
//     messages: Vec<ChatMessage>,
// ) -> Result<ChatResponse> {
//     // Get the model name
//     let model_name = chat_model_name_to_string(&model.name)?;
// }