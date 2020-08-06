#[cfg(test)]
mod test {
    use crate::models::backend::transactions::{Transaction as TransactionDto, ModuleTransaction, EthereumTransaction};
    use crate::providers::info::*;
    use chrono::Utc;
    use crate::models::commons::Operation;
    use crate::models::service::transactions::{TransactionStatus, TransactionInfo, Custom, ID_PREFIX_ETHEREUM_TX, ID_PREFIX_MODULE_TX, ID_PREFIX_MULTISIG_TX, ID_SEPARATOR, Transfer, TransferDirection, TransferInfo, EtherTransfer};
    use crate::models::service::transactions::summary::TransactionSummary;
    use crate::utils::hex_hash;
    use crate::models::backend::transfers::{EtherTransfer as EtherTransferDto, Transfer as TransferDto};

    #[test]
    fn unknown_tx_to_summary_transaction() {
        let unknown_tx = TransactionDto::Unknown;
        let mut mock_info_provider = MockInfoProvider::new();

        let error = unknown_tx.to_transaction_summary(&mut mock_info_provider, &String::from(""));

        assert!(error.is_err());
    }

    #[test]
    fn module_tx_to_summary_transaction() {
        let expected_to = String::from("0x12345789");
        let expected_date = Utc::now();
        let expected_date_in_millis = expected_date.timestamp_millis();
        let module_tx = ModuleTransaction {
            created: String::from("created"),
            execution_date: expected_date,
            block_number: 0,
            transaction_hash: String::from("tx_hash"),
            safe: String::from("safe"),
            module: String::from("module"),
            to: expected_to.clone(),
            value: None,
            data: None,
            data_decoded: None,
            operation: Operation::CALL,
        };

        let actual = ModuleTransaction::to_transaction_summary(&module_tx);
        let expected = vec!(
            TransactionSummary {
                id: create_id!(
                    ID_PREFIX_MODULE_TX,
                    module_tx.safe,
                    module_tx.block_number,
                    hex_hash(&module_tx)
                ),
                timestamp: expected_date_in_millis,
                tx_status: TransactionStatus::Success,
                execution_info: None,
                tx_info: TransactionInfo::Custom(
                    Custom {
                        to: expected_to,
                        data_size: String::from("0"),
                        value: String::from("0"),
                    }),
            });
        assert_eq!(actual, expected);
    }

    #[test]
    fn ethereum_tx_to_summary_transaction_no_transfers() {
        let safe_address = String::from("0x2323");
        let mut mock_info_provider = MockInfoProvider::new();

        let ethereum_tx = EthereumTransaction {
            execution_date: Utc::now(),
            to: String::from("0x1234"),
            data: None,
            tx_hash: String::from("0x4321"),
            block_number: 0,
            transfers: None,
            from: String::from("0x6789"),
        };

        let actual = EthereumTransaction::to_transaction_summary(&ethereum_tx, &mut mock_info_provider, &safe_address);
        assert_eq!(actual, Vec::new());
    }

    #[test]
    fn ethereum_tx_to_summary_transaction_with_transfers() {
        let safe_address = String::from("0x2323");
        let mut mock_info_provider = MockInfoProvider::new();
        let timestamp = Utc::now();
        let timestamp_millis = timestamp.timestamp_millis();

        let transfers = vec!(
            TransferDto::Ether(EtherTransferDto {
                execution_date: timestamp,
                block_number: 0,
                transaction_hash: "".to_string(),
                to: "".to_string(),
                value: String::from("1"),
                from: "".to_string(),
            }),
            TransferDto::Ether(EtherTransferDto {
                execution_date: timestamp,
                block_number: 0,
                transaction_hash: "".to_string(),
                to: "".to_string(),
                value: String::from("1"),
                from: "".to_string(),
            })
        );
        let ethereum_tx = EthereumTransaction {
            execution_date: timestamp,
            to: String::from("0x1234"),
            data: None,
            tx_hash: String::from("0x4321"),
            block_number: 0,
            transfers: Some(transfers.to_vec()),
            from: String::from("0x6789"),
        };

        let actual = EthereumTransaction::to_transaction_summary(&ethereum_tx, &mut mock_info_provider, &safe_address);
        let expected = vec!(
            TransactionSummary {
                id: create_id!(
                        ID_PREFIX_ETHEREUM_TX,
                        safe_address,
                        ethereum_tx.block_number,
                        hex_hash(&ethereum_tx.transfers.as_ref().unwrap().get(0).unwrap())
                    ),
                timestamp: timestamp_millis,
                tx_status: TransactionStatus::Success,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "".to_string(),
                    recipient: "".to_string(),
                    direction: TransferDirection::Unknown,
                    transfer_info: TransferInfo::Ether(
                        EtherTransfer {
                            value: "1".to_string(),
                        }),
                }),
                execution_info: None,
            }, TransactionSummary {
                id: create_id!(
                        ID_PREFIX_ETHEREUM_TX,
                        safe_address,
                        ethereum_tx.block_number,
                        hex_hash(&ethereum_tx.transfers.as_ref().unwrap().get(1).unwrap())
                    ),
                timestamp: timestamp_millis,
                tx_status: TransactionStatus::Success,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "".to_string(),
                    recipient: "".to_string(),
                    direction: TransferDirection::Unknown,
                    transfer_info: TransferInfo::Ether(
                        EtherTransfer {
                            value: "1".to_string(),
                        }),
                }),
                execution_info: None,
            });
        assert_eq!(actual, expected);
    }
}