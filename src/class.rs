
struct ClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    fields_count: u16,
    methods_count: u16,
    attributes_count: u16
}

struct ConstantPoolInfo {
    tag: u1,
    info: Vec<u8>
}

enum ConstantPoolTag {
    CLASS              = 7,
    FIELDREF           = 9,
    METHODREF          = 10,
    INTERFACEMETHODREF = 11,
    STRING             = 8,
    INTEGER            = 3,
    FLOAT              = 4,
    LONG               = 5,
    DOUBLE             = 6,
    NAMEANDTYPE        = 12,
    UTF8               = 1,
    METHODHANDLE       = 15,
    METHODTYPE         = 16,
    INVOKEDYNAMIC      = 18
}

enum ClassAccessFlag {
    ACC_PUBLIC     = 0x0001,
    ACC_FINAL      = 0x0010,
    ACC_SUPER      = 0x0020,
    ACC_INTERFACE  = 0x0200,
    ACC_ABSTRACT   = 0x0400,
    ACC_SYNTHETIC  = 0x1000,
    ACC_ANNOTATION = 0x2000,
    ACC_ENUM       = 0x4000
}

struct FieldInfo {
    access_flags: u2,
    name_index: u2,
    descriptor_index: u2,
    attributes_count: u2
}

enum FieldAccessFlag {

}

enum MethodAccessFlag {

}


