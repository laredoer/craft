package example

import (
	"fmt"
	utils "github.com/wule61/derive/utils"
)

var ProductIDIsEmpty_ ProductIDIsEmpty = 400
var productidisemptyLocales = map[string]string{
  "code": "400",
  "zh-HK": "參數錯誤，請檢查商品編號是否為空",
  "zh-CN": "参数错误，请检查商品编号是否为空",
  "en": "Parameter error, please check if the product number is empty",
}

func (ProductIDIsEmpty) Code() int32 {
  return 400
}

func (ProductIDIsEmpty) Trans(langOrArgs ...any) string {
	lang, args := utils.ParseLangArgs(langOrArgs...)
	if msg, ok := productidisemptyLocales[lang]; ok {
		if len(args) > 0 {
			return fmt.Sprintf(msg, args...)
		}
		return msg
	}
	return fmt.Sprintf(productidisemptyLocales["zh-HK"], args...)
}

var StripePaymentIDError_ StripePaymentIDError = 400
var stripepaymentiderrorLocales = map[string]string{
  "code": "400",
  "zh-HK": "參數錯誤，stripe 支付編號不能為空",
  "zh-CN": "参数错误，stripe 支付编号不能为空",
  "en": "Parameter error, stripe payment number cannot be empty",
}

func (StripePaymentIDError) Code() int32 {
  return 400
}

func (StripePaymentIDError) Trans(langOrArgs ...any) string {
	lang, args := utils.ParseLangArgs(langOrArgs...)
	if msg, ok := stripepaymentiderrorLocales[lang]; ok {
		if len(args) > 0 {
			return fmt.Sprintf(msg, args...)
		}
		return msg
	}
	return fmt.Sprintf(stripepaymentiderrorLocales["zh-HK"], args...)
}

