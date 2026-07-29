// pub static SETTINGS_TEST: LazyLock<fn(&str, &str) -> SETTINGS> = LazyLock::new(|| {

//     |key: &str, secret: &str| SETTINGS {
//         exch: SETTINGS_EXCH {
//             url: "https://api-demo.bybit.com".to_string(),
//             key: key.to_string(),
//             secret: secret.to_string(),
//             timeout_req_ms: 30,
//             timeout_cycle_ms: 31,
//         },
//         files_path: Default::default(),
//         trade: SETTINGS_TRADE {
//             klines_qty: 10_000,
//             leverage: 10.,
//             coins_black_list: vec!["USDC".to_string()],
//             ..Default::default()
//         },
//         symbols_filters: vec![SETTINGS_SYMBOL_FILTER {
//             key: "ordering".to_string(),
//             kwargs_f64: MAP::from_iter([("value".to_string(), 1.3)]),
//             kwargs_string: MAP::from_iter([("type_".to_string(), "greater".to_string())]),
//             used_ind_stat_values: vec!["profit_factor".to_string()],
//             ..Default::default()
//         }],
//         indications: (),
//         signals_train: (),
//         signals: (),
//         order_creators: (),
//         order_collectors: (),
//         order_filters: (),
//         utils_state: (),
//         indications_stat_values: (),
//         indications_stat_columns: (),
//     }
// });
