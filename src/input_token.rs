use dialoguer::Input;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

const TOKEN_PATH: &str = "TOKEN";

pub async fn input_or_get_token() -> Result<String, tokio::io::Error> {
    let f = File::open(TOKEN_PATH).await;

    let token = match f {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s).await?;
            s
        }
        Err(_) => {
            let s: String = Input::new()
                .with_prompt("Input your bot TOKEN")
                .interact_text()
                .unwrap();

            let mut created_file = File::create(TOKEN_PATH).await?;
            created_file.write_all(s.as_bytes()).await?;

            s
        }
    };

    Ok(token)
}
