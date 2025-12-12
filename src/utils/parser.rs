use crate::{errors::ServiceError, models::todos::InsertableTodo};
use bytes::Buf;
use futures_util::TryStreamExt;

pub async fn process_json(
    data: warp::multipart::FormData,
) -> Result<Vec<InsertableTodo>, ServiceError> {
    let result = extract_file_content(data).await?;

    let to_insertable: Vec<InsertableTodo> =
        serde_json::from_str(&result).map_err(|_| ServiceError::BadRequest)?;

    Ok(to_insertable)
}

pub async fn process_csv(
    data: warp::multipart::FormData,
) -> Result<Vec<InsertableTodo>, ServiceError> {
    let result = extract_file_content(data).await?;

    let lines = result.split('\n').collect::<Vec<&str>>();

    let to_insertable = lines
        .iter()
        .skip(1)
        .filter_map(|line| {
            let cols = line.split(',').collect::<Vec<&str>>();

            if cols.len() >= 2 {
                Some(InsertableTodo {
                    title: cols[0].trim().to_string(),
                    body: cols[1].trim().to_string(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<InsertableTodo>>();

    Ok(to_insertable)
}

async fn read_part_data(part: warp::multipart::Part) -> Result<Vec<u8>, ServiceError> {
    part.stream()
        .try_fold(Vec::new(), |mut acc, chunk| async move {
            acc.extend_from_slice(chunk.chunk());

            Ok(acc)
        })
        .await
        .map_err(|_| ServiceError::BadRequest)
}

async fn extract_file_content(mut data: warp::multipart::FormData) -> Result<String, ServiceError> {
    let mut file_content: Option<String> = None;

    while let Ok(Some(part)) = data.try_next().await {
        if part.name() == "file" {
            let data = read_part_data(part).await?;

            file_content = Some(String::from_utf8(data).map_err(|_| ServiceError::BadRequest)?);

            break;
        }
    }

    match file_content {
        Some(content) => Ok(content),
        None => Err(ServiceError::BadRequest),
    }
}
