use super::serde_url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ImageAttributes {
    size: usize,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Data {
    #[serde(rename = "source")]
    Source { url: Url },
    #[serde(rename = "input")]
    Input(ImageAttributes),
    #[serde(rename = "output")]
    Output(ImageAttributes),
    #[serde(rename = "resize")]
    Resize {
        method: String,
        width: Option<usize>,
        height: Option<usize>,
    },
}

#[cfg(test)]
mod test {
    use super::Data;
    use super::ImageAttributes;
    use Url;
    use serde_json;

    #[test]
    fn test_data_serde_input() {
        let data = Data::Input(ImageAttributes {
            size: 258,
            type_: "image/png".into(),
        });
        let json = serde_json::to_string(&data).unwrap();
        debug!("json: {}", json);
        assert_eq!(json, r#"{"input":{"size":258,"type":"image/png"}}"#);
        let new_data = serde_json::from_str(&json).unwrap();
        println!("data: {:?}", new_data);
        assert_eq!(data, new_data);
    }

    #[test]
    fn test_data_serde_output() {
        let data = Data::Output(ImageAttributes {
            size: 258,
            type_: "image/png".into(),
        });
        let json = serde_json::to_string(&data).unwrap();
        debug!("json: {}", json);
        assert_eq!(json, r#"{"output":{"size":258,"type":"image/png"}}"#);
        let new_data = serde_json::from_str(&json).unwrap();
        println!("data: {:?}", new_data);
        assert_eq!(data, new_data);
    }

    #[test]
    fn test_data_serde_source() {
        let data = Data::Source {
            url: Url::parse("https://test.com/image.png").unwrap(),
        };
        let json = serde_json::to_string(&data).unwrap();
        debug!("json: {}", json);
        assert_eq!(json, r#"{"source":{"url":"https://test.com/image.png"}}"#);
        let new_data = serde_json::from_str(&json).unwrap();
        println!("data: {:?}", new_data);
        assert_eq!(data, new_data);
    }
    #[test]
    fn test_data_serde_resize() {
        let data = Data::Resize {
            method: "fit".into(),
            width: Some(150),
            height: Some(100),
        };
        let json = serde_json::to_string(&data).unwrap();
        debug!("json: {}", json);
        assert_eq!(
            json,
            r#"{"resize":{"method":"fit","width":150,"height":100}}"#
        );
        let new_data = serde_json::from_str(&json).unwrap();
        println!("data: {:?}", new_data);
        assert_eq!(data, new_data);
    }

}
