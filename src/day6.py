main_data = []
with open('../input/day6.txt') as f:
    data = f.read()
    main_data = list(map(int, data.split(",")))

for _ in range(0,256):
    for i in range(0,len(main_data)):
        if main_data[i] == 0:
            main_data.append(8)
            main_data[i] = 6
            continue
        
        main_data[i] -= 1

print(len(main_data))

