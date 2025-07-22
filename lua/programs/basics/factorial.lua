#!/usr/bin/env lua

function Factorial(n)
	if n <= 0 then
		return 1
	end

	return n * Factorial(n - 1)
end

function Main()
	if arg[1] == nil then
		print("Usage: " .. arg[0] .. " <number>")
		return
	end

	local num = tonumber(arg[1])

	if type(num) ~= "number" then
		print("Expected number. Got: " .. arg[1])
		return
	end

	if num < 0 then
		print("Non-negative number expected. Got: " .. num)
		return
	end

	print(num .. "! = " .. Factorial(num))
end

Main()
