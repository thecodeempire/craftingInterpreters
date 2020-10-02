import common_types

proc newReturnError*(value: Literal): ReturnError=
 return ReturnError(value: value, message: "", token: DefaultToken)

