function Factorial(n)
	if n == 0 then
		return 1
	else
		return n * Factorial(n - 1)
	end
end

function Main()
	print("Enter a number: ")
	local a = io.read("*n")
	print(Factorial(a))
end

Main()
