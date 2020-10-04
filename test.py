#!/usr/bin/python3

import sys
import subprocess
import os

if len(sys.argv) != 4:
	print("Error: Insufficient arguments.")
	print("Syntax: test.py <test source> <test bin> <test_type>")
	exit()
	
test_file = sys.argv[1]
bin_file = sys.argv[2]
test_type = sys.argv[3]

basename = os.path.basename(test_file)
print("[TEST] " + basename)

output = []
in_output = False
ret = 0

with open(test_file) as reader:
	for ln in reader:
		ln = ln.strip()
		
		if ln == "#OUTPUT":
			in_output = True
		elif ln == "#END" and in_output:
			in_output = False
		elif ln.startswith("#RET"):
			ret = int(ln.split()[1])
		elif in_output:
			output.append(ln[1:])
			
result = subprocess.run([bin_file], stdout=subprocess.PIPE)
cmd_output = result.stdout.decode('utf-8').split('\n')
cmd_output.remove('')
rc = result.returncode

# Check output
is_output = True

if len(output) != len(cmd_output):
	is_output = False
	
if is_output:
	for i in range(len(output)):
		if output[i] != cmd_output[i]:
			is_output = False
			break
			
is_ret = True
if test_type != "error":
	if rc != ret:
		is_ret = False

# Print results if wrong
if (not is_ret) or (not is_output):
	print("Expected Output: " + str(output))
	print("CMD Output: " + str(cmd_output))
	print("")
	if test_type != "error":
		print("Expected Return: " + str(ret))
		print("Actual Return: " + str(rc))
		print("")
	print("Fail")
	exit(1)
	
print("Pass")
print("")

exit(0)

