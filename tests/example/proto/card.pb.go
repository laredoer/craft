// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        v3.21.12
// source: proto/card.proto

package proto

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// #[Validate]
type GetCardInfoReq struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// #[Validate(required)]
	// #[i18n(zh-HK = "參數錯誤，卡券編號不能為空", zh-CN = "参数错误，卡券编号不能为空", en = "Parameter error, Card id cannot be empty")]
	CardId string `protobuf:"bytes,1,opt,name=card_id,json=cardId,proto3" json:"card_id,omitempty"`
	// #[Validate(required)]
	// #[i18n(zh-HK = "參數錯誤，名字不能為空", zh-CN = "参数错误，名字不能为空", en = "Parameter error, name cannot be empty")]
	FirstName string `protobuf:"bytes,2,opt,name=first_name,json=firstName,proto3" json:"first_name,omitempty"`
	LastName  string `protobuf:"bytes,3,opt,name=last_name,json=lastName,proto3" json:"last_name,omitempty"`
	// #[Validate(required, gte=0, lte=130)]
	// #[i18n(zh-HK = "參數錯誤，年齡必須在 0-130 之間", zh-CN = "参数错误，年龄必须在 0-130 之间", en = "Parameter error, age must be between 0-130")]
	Age uint32 `protobuf:"varint,4,opt,name=age,proto3" json:"age,omitempty"`
	// #[Validate(required, email)]
	// #[i18n(zh-HK = "參數錯誤，郵箱格式錯誤", zh-CN = "参数错误，邮箱格式错误", en = "Parameter error, email format error")]
	Email string `protobuf:"bytes,5,opt,name=email,proto3" json:"email,omitempty"`
	// #[Validate(oneof="male female prefer_not_to")]
	Gender string `protobuf:"bytes,6,opt,name=gender,proto3" json:"gender,omitempty"`
	// #[Validate(iscolor)]
	// #[i18n(zh-HK = "參數錯誤，顏色格式錯誤", zh-CN = "参数错误，颜色格式错误", en = "Parameter error, color format error")]
	Color string `protobuf:"bytes,7,opt,name=color,proto3" json:"color,omitempty"`
}

func (x *GetCardInfoReq) Reset() {
	*x = GetCardInfoReq{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_card_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetCardInfoReq) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetCardInfoReq) ProtoMessage() {}

func (x *GetCardInfoReq) ProtoReflect() protoreflect.Message {
	mi := &file_proto_card_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetCardInfoReq.ProtoReflect.Descriptor instead.
func (*GetCardInfoReq) Descriptor() ([]byte, []int) {
	return file_proto_card_proto_rawDescGZIP(), []int{0}
}

func (x *GetCardInfoReq) GetCardId() string {
	if x != nil {
		return x.CardId
	}
	return ""
}

func (x *GetCardInfoReq) GetFirstName() string {
	if x != nil {
		return x.FirstName
	}
	return ""
}

func (x *GetCardInfoReq) GetLastName() string {
	if x != nil {
		return x.LastName
	}
	return ""
}

func (x *GetCardInfoReq) GetAge() uint32 {
	if x != nil {
		return x.Age
	}
	return 0
}

func (x *GetCardInfoReq) GetEmail() string {
	if x != nil {
		return x.Email
	}
	return ""
}

func (x *GetCardInfoReq) GetGender() string {
	if x != nil {
		return x.Gender
	}
	return ""
}

func (x *GetCardInfoReq) GetColor() string {
	if x != nil {
		return x.Color
	}
	return ""
}

type GetCardInfoResp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *GetCardInfoResp) Reset() {
	*x = GetCardInfoResp{}
	if protoimpl.UnsafeEnabled {
		mi := &file_proto_card_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetCardInfoResp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetCardInfoResp) ProtoMessage() {}

func (x *GetCardInfoResp) ProtoReflect() protoreflect.Message {
	mi := &file_proto_card_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetCardInfoResp.ProtoReflect.Descriptor instead.
func (*GetCardInfoResp) Descriptor() ([]byte, []int) {
	return file_proto_card_proto_rawDescGZIP(), []int{1}
}

var File_proto_card_proto protoreflect.FileDescriptor

var file_proto_card_proto_rawDesc = []byte{
	0x0a, 0x10, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x63, 0x61, 0x72, 0x64, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x05, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xbb, 0x01, 0x0a, 0x0e, 0x47, 0x65,
	0x74, 0x43, 0x61, 0x72, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x71, 0x12, 0x17, 0x0a, 0x07,
	0x63, 0x61, 0x72, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x63,
	0x61, 0x72, 0x64, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x66, 0x69, 0x72, 0x73, 0x74, 0x5f, 0x6e,
	0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x66, 0x69, 0x72, 0x73, 0x74,
	0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x6e, 0x61, 0x6d,
	0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6c, 0x61, 0x73, 0x74, 0x4e, 0x61, 0x6d,
	0x65, 0x12, 0x10, 0x0a, 0x03, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x03,
	0x61, 0x67, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x05, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x12, 0x16, 0x0a, 0x06, 0x67, 0x65, 0x6e,
	0x64, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x67, 0x65, 0x6e, 0x64, 0x65,
	0x72, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x22, 0x11, 0x0a, 0x0f, 0x47, 0x65, 0x74, 0x43, 0x61,
	0x72, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x32, 0x44, 0x0a, 0x04, 0x43, 0x61,
	0x72, 0x64, 0x12, 0x3c, 0x0a, 0x0b, 0x47, 0x65, 0x74, 0x43, 0x61, 0x72, 0x64, 0x49, 0x6e, 0x66,
	0x6f, 0x12, 0x15, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x47, 0x65, 0x74, 0x43, 0x61, 0x72,
	0x64, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x71, 0x1a, 0x16, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x2e, 0x47, 0x65, 0x74, 0x43, 0x61, 0x72, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x73, 0x70,
	0x42, 0x12, 0x5a, 0x10, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2f, 0x67, 0x6f, 0x2f, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_proto_card_proto_rawDescOnce sync.Once
	file_proto_card_proto_rawDescData = file_proto_card_proto_rawDesc
)

func file_proto_card_proto_rawDescGZIP() []byte {
	file_proto_card_proto_rawDescOnce.Do(func() {
		file_proto_card_proto_rawDescData = protoimpl.X.CompressGZIP(file_proto_card_proto_rawDescData)
	})
	return file_proto_card_proto_rawDescData
}

var file_proto_card_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_proto_card_proto_goTypes = []interface{}{
	(*GetCardInfoReq)(nil),  // 0: proto.GetCardInfoReq
	(*GetCardInfoResp)(nil), // 1: proto.GetCardInfoResp
}
var file_proto_card_proto_depIdxs = []int32{
	0, // 0: proto.Card.GetCardInfo:input_type -> proto.GetCardInfoReq
	1, // 1: proto.Card.GetCardInfo:output_type -> proto.GetCardInfoResp
	1, // [1:2] is the sub-list for method output_type
	0, // [0:1] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_proto_card_proto_init() }
func file_proto_card_proto_init() {
	if File_proto_card_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_proto_card_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetCardInfoReq); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_proto_card_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetCardInfoResp); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_proto_card_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_proto_card_proto_goTypes,
		DependencyIndexes: file_proto_card_proto_depIdxs,
		MessageInfos:      file_proto_card_proto_msgTypes,
	}.Build()
	File_proto_card_proto = out.File
	file_proto_card_proto_rawDesc = nil
	file_proto_card_proto_goTypes = nil
	file_proto_card_proto_depIdxs = nil
}