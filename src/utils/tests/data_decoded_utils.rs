use crate::models::commons::{
    DataDecoded, InternalTransaction, Operation, ParamValue, Parameter, ValueDecodedType,
};

#[test]
fn get_parameter_single_value_success() {
    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_ADD_OWNER_WITH_THRESHOLD)
            .unwrap();
    let expected = "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23";

    let actual = data_decoded.get_parameter_single_value("owner");

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn get_parameter_single_value_wrong_name() {
    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_ADD_OWNER_WITH_THRESHOLD)
            .unwrap();
    let expected = None;

    let actual = data_decoded.get_parameter_single_value("threshold");

    assert_eq!(expected, actual);
}

#[test]
fn get_parameter_single_value_right_name_but_array_value() {
    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_SWAP_ARRAY_VALUES).unwrap();
    let expected = None;

    let actual = data_decoded.get_parameter_single_value("data");

    assert_eq!(expected, actual);
}

#[test]
fn get_parameter_value_decoded_success() {
    let data_decoded = serde_json::from_str::<DataDecoded>(
        crate::json::DATA_DECODED_MULTI_SEND_SINGLE_INNER_TRANSACTION,
    )
    .unwrap();

    let expected = ValueDecodedType::InternalTransaction(vec![InternalTransaction {
        operation: Operation::CALL,
        to: "0x111111125434b319222CdBf8C261674aDB56F3ae".to_string(),
        value: Some(22 as u64),
        data: Some("0x90411a32000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000001c0000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000bc79855178842fdba0c353494895deef509e26bb000000000000000000000000000000000000000000000ed2b525841adfc00000000000000000000000000000000000000000000000000ecee9b38efb1a680000000000000000000000000000000000000000000000000ed2b525841adfc000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000440000000000000000000000000000000000000000000000000000000000000076000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000324b3af37c000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000024000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000000000001400000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000001e45636885000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000bc79855178842fdba0c353494895deef509e26bb000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000000000000000000000000ecee9b38efb1a68000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ed2b525841adfc0000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000004d0e30db0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000002647f8fe7a000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000044000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe500000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000a405971224000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000000000000000000000002f9ae7c8305c3600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004470bdb947000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000ed2b525841adfc00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000184b3af37c000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000024000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000100000000000000000000000000000001000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000044a9059cbb000000000000000000000000bc79855178842fdba0c353494895deef509e26bb00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()),
        data_decoded: Some(DataDecoded{
            method: "swap".to_string(),
            parameters: Some(vec![
                Parameter{
                    name: "caller".to_string(),
                    param_type: "address".to_string(),
                    value: ParamValue::SingleValue("0xd47140F6Ab73f6d6B6675Fb1610Bb5E9B5d96FE5".to_string()),
                    value_decoded: None
                },
                Parameter{
                    name: "desc".to_string(),
                    param_type: "(address,address,address,address,uint256,uint256,uint256,uint256,address,bytes)".to_string(),
                    value: ParamValue::ArrayValue(vec![
                        ParamValue::SingleValue("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string()),
                        ParamValue::SingleValue("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string()),
                        ParamValue::SingleValue("0xd47140F6Ab73f6d6B6675Fb1610Bb5E9B5d96FE5".to_string()),
                        ParamValue::SingleValue("0xBc79855178842FDBA0c353494895DEEf509E26bB".to_string()),
                        ParamValue::SingleValue("70000000000000000000000".to_string()),
                        ParamValue::SingleValue("69930000000000000000000".to_string()),
                        ParamValue::SingleValue("70000000000000000000000".to_string()),
                        ParamValue::SingleValue("1".to_string()),
                        ParamValue::SingleValue("0x0000000000000000000000000000000000000000".to_string()),
                        ParamValue::SingleValue("0x".to_string())
                    ]),
                    value_decoded: None
                },
                Parameter{
                    name: "calls".to_string(),
                    param_type: "(uint256,uint256,uint256,bytes)[]".to_string(),
                    value: ParamValue::ArrayValue(
                        vec![
                            ParamValue::ArrayValue(vec![
                                ParamValue::SingleValue("0".to_string()),
                                ParamValue::SingleValue("0".to_string()),
                                ParamValue::SingleValue("0".to_string()),
                                ParamValue::SingleValue("0xb3af37c000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000024000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000000000001400000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000001e45636885000000000000000000000000000000000000000000000000000000000000000e00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000bc79855178842fdba0c353494895deef509e26bb000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe5000000000000000000000000000000000000000000000ecee9b38efb1a68000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ed2b525841adfc0000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000004d0e30db00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()),
                            ]),
                            ParamValue::ArrayValue(vec![
                                ParamValue::SingleValue("0".to_string()),
                                ParamValue::SingleValue("0".to_string()),
                                ParamValue::SingleValue("0".to_string()),
                                ParamValue::SingleValue("0x7f8fe7a000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000044000000000000000000000000d47140f6ab73f6d6b6675fb1610bb5e9b5d96fe500000000000000000000000000000000000000000000000000000000000001e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000a405971224000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000100000000000000000000000000000001000000000000000000000000000000000000000000000000002f9ae7c8305c3600000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004470bdb947000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000ed2b525841adfc0000000000000000000000000000000000000000000000000000000000000".to_string()),
                            ]),
                            ParamValue::ArrayValue(vec![
                                ParamValue::SingleValue("0".to_string()),
                                ParamValue::SingleValue("0".to_string()),
                                ParamValue::SingleValue("0".to_string()),
                                ParamValue::SingleValue("0xb3af37c000000000000000000000000000000000000000000000000000000000000000808000000000000000000000000000000000000000000000000000000000000024000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000100000000000000000000000000000001000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000044a9059cbb000000000000000000000000bc79855178842fdba0c353494895deef509e26bb000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000".to_string()),
                            ]),
                        ]
                    ),
                    value_decoded: None
                }
            ]),
        }),
    }]);

    let actual = data_decoded.get_parameter_value_decoded("transactions");

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn get_parameter_value_decoded_wrong_name() {
    let data_decoded = serde_json::from_str::<DataDecoded>(
        crate::json::DATA_DECODED_MULTI_SEND_SINGLE_INNER_TRANSACTION,
    )
    .unwrap();
    let expected = None;

    let actual = data_decoded.get_parameter_single_value("wrong_transactions");

    assert_eq!(expected, actual);
}

#[test]
fn get_parameter_value_decoded_right_name_but_array_value() {
    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_SWAP_ARRAY_VALUES).unwrap();
    let expected = None;

    let actual = data_decoded.get_parameter_value_decoded("swap");

    assert_eq!(expected, actual);
}

#[test]
fn get_action_count_multisig_call() {
    let data_decoded_action_count_1 = serde_json::from_str::<DataDecoded>(
        crate::json::DATA_DECODED_MULTI_SEND_SINGLE_INNER_TRANSACTION,
    )
    .unwrap();

    let data_decoded_action_count_3 =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_MULTI_SEND).unwrap();

    assert_eq!(Some(1), data_decoded_action_count_1.get_action_count());
    assert_eq!(Some(3), data_decoded_action_count_3.get_action_count());
}

#[test]
fn get_action_count_non_multisig_call() {
    let data_decoded = serde_json::from_str::<DataDecoded>(
        crate::json::DATA_DECODED_EXEC_TRANSACTION_WITH_VALUE_DECODED,
    )
    .unwrap();
    let expected = None;

    let actual = data_decoded.get_parameter_value_decoded("data");

    assert_eq!(expected, actual);
}