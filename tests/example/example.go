package example

// #[i18n(code = 400, zh-HK = "參數錯誤，請檢查商品編號是否為空", zh-CN = "参数错误，请检查商品编号是否为空", en = "Parameter error, please check if the product number is empty")]
type ProductIDIsEmpty int32

// #[i18n(code = 400, zh-HK = "參數錯誤，stripe 支付編號不能為空", zh-CN = "参数错误，stripe 支付编号不能为空", en = "Parameter error, stripe payment number cannot be empty")]
// #[Debug]
type StripePaymentIDError int32
