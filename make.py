# TODO:
# Rest

def Read(path):
	read_path = path
	write_path = "" # TBD
	err = 1
	
	# defaults
	kernel = 1
	bootable = 1
	gui = 0
	shell = 0
	wm = 0
	bare_func = 0
	
	x = 1
	while x == 1:
		# get a line from the file
			
		if line == "bare functions true":
			err = 0
			bare_func = 1
		
		if line == "gui true":
			err = 0
			gui = 1
			
