package example

// #[Validate]
type Hello2 struct {
	// 参数错误，名称不能为空
	// #[Validate(required,email,gte=0,lte=130, iscolor, oneof="male female prefer_not_to")]
	// #[i18n(code = 400, zh-HK = "參數錯誤，名稱不能為空", zh-CN = "参数错误，名称不能为空", en = "Parameter error, name cannot be empty")]
	Name string

	Age int64
}
