import pprint, openpyxl
print('Opening workbook')

wb = openpyxl.load_workbook('censuspopdata.xlsx')
sheet = wb['Population by Census Tract']
country_data = {}

print('Reading rows')
for row in range(2, sheet.max_row+1):
    state  = sheet['B' + str(row)].value
    country = sheet['C' + str(row)].value
    pop    = sheet['D' + str(row)].value

    country_data.setdefault(state, {})
    country_data[state].setdefault(country, {'tracts' : 0, 'pop' : 0})

    country_data[state][country]['tracts'] += 1
    country_data[state][country]['pop'] += int(pop)

print('Writing rows')
result_file = open('census2010.py', 'w')
result_file.write('allData = '+ pprint.pformat(country_data))
print('Done.')
