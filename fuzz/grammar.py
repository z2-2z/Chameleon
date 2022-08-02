# Grammar for nautilus-fuzz #
#############################

C = 0

def next_struct(b):
    global C
    name = b"S%d" % C
    if C == 0:
        name = b"Root"
    C += 1
    return b"struct " + name + b

def reset_struct(opts, structs=b""):
    global C
    C = 0
    return opts + structs

def ref_struct():
    global C
    if C <= 1:
        return b"Root"
    else:
        return b"S%d" % (C - 1)

#ctx.script("START", ["Options", "Structs"], reset_struct)
ctx.script("START", ["Structs"], reset_struct)
ctx.rule("Options", "{Option}{Options}")
ctx.rule("Options", "")
ctx.rule("Option", "option endianness={Endianness};\n")
ctx.rule("Option", "option scheduling={Scheduling};\n")
ctx.rule("Endianness", "little")
ctx.rule("Endianness", "big")
ctx.rule("Endianness", "native")
ctx.rule("Scheduling", "random")
ctx.rule("Scheduling", "round-robin")
ctx.rule("Structs", "{Struct}{Structs}")
ctx.rule("Structs", "")
ctx.script("Struct", ["Block"], next_struct)
#ctx.rule("Block", "\{\n{Options}{VariableListing}\}")
ctx.rule("Block", "\{\n{VariableListing}\}")
ctx.rule("VariableListing", "{Variable}{VariableListing}")
ctx.rule("VariableListing", "")
ctx.rule("Variable", "{VariableFlags} _:{VariableDef};\n")
ctx.script("VariableDef", [], ref_struct)
ctx.rule("VariableDef", "{NumericalVar}") 
ctx.rule("VariableDef", "{BytearrayVar}")
ctx.rule("VariableDef", "struct {Block}")
ctx.rule("VariableDef", "oneof {Block}")
ctx.rule("BytearrayVar", "{BytearrayType}={BytearrayValue}")
ctx.rule("BytearrayValue", "{Numberset}")
ctx.rule("BytearrayValue", "{StringLiteral}")
ctx.rule("StringLiteral", "\"{Char}{Char}{Char}\"")

for i in range(0x20, 0x22):
    ctx.rule("Char", chr(i))
for i in range(0x23, 0x27):
    ctx.rule("Char", chr(i))
for i in range(0x28, 0x5C):
    ctx.rule("Char", chr(i))
for i in range(0x5D, 0x7F):
    ctx.rule("Char", chr(i))
for i in ["r", "n", "t", "0", "a", "b", "f", "v", "x"]:
    ctx.rule("Char", "\\" + i)

ctx.rule("NumericalVar", "{NumericalType}{NumericalValue}")
ctx.rule("NumericalValue", "")
ctx.rule("NumericalValue", "={Numberset}")
ctx.rule("VariableFlags", "{OptionalFlag} {RepeatsFlag} ")
ctx.rule("OptionalFlag", "optional")
ctx.rule("OptionalFlag", "")
ctx.rule("RepeatsFlag", "repeats {Numberset}")
ctx.rule("RepeatsFlag", "")
ctx.rule("Numberset", "{NumbersetElem}{NextNumbersetElem}")
ctx.rule("NextNumbersetElem", ",{NumbersetElem}{NextNumbersetElem}")
ctx.rule("NextNumbersetElem", "")
ctx.rule("NumbersetElem", "{Integer}")
ctx.rule("NumbersetElem", "{Range}")
ctx.rule("NumbersetElem", "{Bitpattern}")
ctx.rule("NumbersetElem", "'{Char}'")
ctx.rule("Range", "{Integer}..{Integer}")
ctx.rule("Range", "{Bitpattern}..{Bitpattern}")
ctx.rule("Integer", "{Sign}{DecDigit}{NextDecDigit}")
ctx.rule("Sign", "-")
ctx.rule("Sign", "")
ctx.rule("DecDigit", "{OctDigit}")
ctx.rule("DecDigit", "8")
ctx.rule("DecDigit", "9")
ctx.rule("NextDecDigit", "{DecDigit}{NextDecDigit}")
ctx.rule("NextDecDigit", "")
ctx.rule("Bitpattern", "0x{HexDigit}{NextHexDigit}")
ctx.rule("Bitpattern", "0o{OctDigit}{NextOctDigit}")
ctx.rule("Bitpattern", "0b{BinDigit}{NextBinDigit}")
ctx.rule("HexDigit", "{DecDigit}")
ctx.rule("HexDigit", "A")
ctx.rule("HexDigit", "B")
ctx.rule("HexDigit", "C")
ctx.rule("HexDigit", "D")
ctx.rule("HexDigit", "E")
ctx.rule("HexDigit", "F")
ctx.rule("HexDigit", "a")
ctx.rule("HexDigit", "b")
ctx.rule("HexDigit", "c")
ctx.rule("HexDigit", "d")
ctx.rule("HexDigit", "e")
ctx.rule("HexDigit", "f")
ctx.rule("NextHexDigit", "{HexDigit}{NextHexDigit}")
ctx.rule("NextHexDigit", "")
ctx.rule("OctDigit", "{BinDigit}")
ctx.rule("OctDigit", "2")
ctx.rule("OctDigit", "3")
ctx.rule("OctDigit", "4")
ctx.rule("OctDigit", "5")
ctx.rule("OctDigit", "6")
ctx.rule("OctDigit", "7")
ctx.rule("NextOctDigit", "{OctDigit}{NextOctDigit}")
ctx.rule("NextOctDigit", "")
ctx.rule("BinDigit", "0")
ctx.rule("BinDigit", "1")
ctx.rule("NextBinDigit", "{BinDigit}{NextBinDigit}")
ctx.rule("NextBinDigit", "")
ctx.rule("NumericalType", "u8")
ctx.rule("NumericalType", "i8")
ctx.rule("NumericalType", "u16")
ctx.rule("NumericalType", "i16")
ctx.rule("NumericalType", "u32")
ctx.rule("NumericalType", "i32")
ctx.rule("NumericalType", "u64")
ctx.rule("NumericalType", "i64")
ctx.rule("NumericalType", "char")
ctx.rule("BytearrayType", "string")
ctx.rule("BytearrayType", "bytes")
