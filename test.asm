; simple test code
Push 10
Push 8
AddStack
PopRegister 0
Push 10
Push 12
PopRegister 1
PopRegister 2
AddRegister 1 2
Signal $f0
