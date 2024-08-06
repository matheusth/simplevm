; simple test code
Push 10
Push 8
AddStack
PopRegister 0
Push 10
PopRegister 1
AddRegister 0 1
Signal $f0
