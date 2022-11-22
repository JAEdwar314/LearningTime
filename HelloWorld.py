#Testing stuff

number = "123456789"

print(number[3::2])


lista = [1,2,3,4,5,6]
listb = ["Jack, John, Bool, Blame"]

for val, valb in zip(lista, listb){
    print(f"{val}", "{valb}")
}