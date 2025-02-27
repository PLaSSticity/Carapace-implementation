import statistics
import scipy.stats as st
import sys

def read_file(file_name):
    f = open(file_name)
    results = []
    for l in f:
        results.append(l)
    return results

def split_input(file_lines):
    file_lines = file_lines[1:]
    file_lines_ifc = file_lines[1::3]
    file_lines_no_ifc = file_lines[2::3]
    return (file_lines_ifc, file_lines_no_ifc)

def trim_input_start(lines):
    for i in range(len(lines)):
        if lines[i][0] == 'I':
            lines[i] = lines[i][13:-1]
        else:
            lines[i] = lines[i][17:-1]
    return lines

def trim_input_end(lines):
    for i in range(len(lines)):
        lines[i] = lines[i][:-2]
    lines = (lines, "µs")
    return lines

def analyze(lines, scale):
    for i in range(len(lines)):
        lines[i] = float(lines[i])
    print("Average: {:.2f}µs, Standard Deviation: {:.2f}µs".format(statistics.mean(lines), statistics.stdev(lines)))
    con_i = st.norm.interval(confidence=0.95, loc=statistics.mean(lines), scale=st.sem(lines))
    print("95% confidence interval: ({:.2f}µs, {:.2f}µs)".format(con_i[0], con_i[1]))

"""lines = read_file("results-with-clone.txt")
lines_ifc, lines_not_ifc = split_input(lines)
lines_ifc, lines_not_ifc = trim_input_end(trim_input_start(lines_ifc)), trim_input_end(trim_input_start(lines_not_ifc))
print("\nIFC with clone trial: ")
analyze(lines_ifc, "m")
print("\nNOT IFC with clone trial: ")
analyze(lines_not_ifc, "ms")
print()


lines = read_file("results-with-vecdeque.txt")
lines_ifc, lines_not_ifc = split_input(lines)
lines_ifc, lines_not_ifc = trim_input_end(trim_input_start(lines_ifc)), trim_input_end(trim_input_start(lines_not_ifc))
print("\nIFC without clone trial: ")
analyze(lines_ifc, "m")
print("\nNOT IFC without clone trial: ")
analyze(lines_not_ifc, "ms")"""

lines = read_file(sys.argv[1])
lines_ifc, lines_not_ifc = split_input(lines)
lines_ifc, lines_not_ifc = trim_input_end(trim_input_start(lines_ifc)), trim_input_end(trim_input_start(lines_not_ifc))
print("\nIFC: ")
analyze(lines_ifc[0], lines_ifc[1])
print("\nNOT IFC: ")
analyze(lines_not_ifc[0], lines_not_ifc[1])