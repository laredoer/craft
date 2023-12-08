package example

// #[i18n(code = 400, zh-HK = "參數錯誤，請檢查商品編號是否為空", zh-CN = "参数错误，请检查商品编号是否为空", en = "Parameter error, please check if the product number is empty")]
type ProductIDIsEmpty int32

// #[i18n(code = 400, zh-HK = "參數錯誤，stripe 支付編號不能為空", zh-CN = "参数错误，stripe 支付编号不能为空", en = "Parameter error, stripe payment number cannot be empty")]
type StripePaymentIDError int32

// #[Validate]
type Hello struct {
	// 参数错误，名称不能为空
	// #[Validate(required,email,gte=0,lte=130, iscolor, oneof="male female prefer_not_to")]
	// #[i18n(zh-HK = "參數錯誤，名稱不能為空", zh-CN = "参数错误，名称不能为空", en = "Parameter error, name cannot be empty")]
	Name string

	// #[Validate(required)]
	Age int64

	Addr string
}
