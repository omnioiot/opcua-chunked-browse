use opcua::client::prelude::*;
use std::path::PathBuf;

fn main() {
    opcua::console_logging::init();
    let mut client_config = ClientConfig {
        application_name: "Omnio Edge Opcua Discovery".into(),
        application_uri: "urn:omnio:edge".into(),
        pki_dir: PathBuf::from("/pki"),
        session_retry_limit: 1,
        session_timeout: 5000,
        certificate_path: Some(PathBuf::from("cert.der")),
        private_key_path: Some(PathBuf::from("private.pem")),
        ..ClientConfig::default()
    };

    client_config.decoding_options.max_chunk_count = i32::MAX as usize;
    client_config.decoding_options.max_string_length = i32::MAX as usize;
    client_config.decoding_options.max_byte_string_length = i32::MAX as usize;
    client_config.decoding_options.max_array_length = i32::MAX as usize;
    client_config.decoding_options.max_message_size = i32::MAX as usize;
    dbg!(&client_config.decoding_options);

    let mut client = Client::new(client_config);
    let uri = "opc.tcp://127.0.0.1:50000";
    let session = client
            .connect_to_endpoint(uri, IdentityToken::Anonymous)
            .unwrap();
    let reader = session.read();
    let browse_desc = 
            BrowseDescription {
                node_id: NodeId::new(2, "Slow"),
                browse_direction: BrowseDirection::Forward,
                reference_type_id: ReferenceTypeId::HierarchicalReferences.into(),
                include_subtypes: true,
                node_class_mask: 0x0,
                result_mask: 0b111111,
            };

    let res = reader.browse(&[browse_desc]).unwrap();
    dbg!(res);
}
