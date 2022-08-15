import matplotlib.pyplot as plt



def parse_txt(file):
    with open(file) as f:
        lines = f.readlines()
    
    if file == "freq.txt":
        start = 1
        end = int(len(lines)/2)
    else:
        start = 0
        end = len(lines)

    x = []
    y = []
    for line in lines[start:end]:
        split = False
        x_tmp = ""
        y_tmp = ""
        for i in range(0, len(line)):
            if line[i] == ",":
                split = True
                continue
            if split == True:
                y_tmp += line[i]
                continue
            x_tmp += line[i]
        x.append(float(x_tmp))
        y.append(float(y_tmp))
    return x, y


def plot_domain(file, xlabel, ylabel):
    x, y = parse_txt(file)
    max_y = max(y)
    idx_max_y = max(range(len(y)), key=y.__getitem__)
    norm_y = []
    print(f'fundamental frequency: {idx_max_y} | magnitude: {max_y}')
    for val in y:
        norm_val = (val - 0) / (max_y) * (1.0 - 0.0) + 0.0
        norm_y.append(norm_val)

    plt.plot(x, norm_y, "r-")
    plt.xlabel(xlabel)
    plt.ylabel(ylabel)
    plt.title(f'{xlabel} domain')

    plt.show()


if __name__ == "__main__":
    domain = "frequency"
    if domain == "frequency":
        plot_domain("freq.txt", "frequency", "magnitude")
    else:
        plot_domain("time.txt", "time", "amplitude")