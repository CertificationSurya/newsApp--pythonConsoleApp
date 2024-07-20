import requests
import json

apiKey = ''
with open("MyApiForNewsApi.key","r") as filePointer:
    apiKey = filePointer.read()


def showNews(response):
    # print(response)
    articles = response['articles']
    for i in range(len(articles)):
        print(f"article {i+1} {articles[i]['title'] } \n")


# For Code Accessed from running code :
print("Enter the category of the news: ")

# creating dict to avoid multiple line of print
categoryDict = ["business","entertainment","general","health","science","sports","technology"]
categoryDict = {index:item for index,item in enumerate(categoryDict)}
for keys,values in categoryDict.items():
    print(f"Enter {keys} for {values} news")


categoryNum = int(input("7 for none \nor 8 for all \n\nYour choice is: "))

if categoryNum == 8:
    response = requests.get(f"https://newsapi.org/v2/top-headlines?country=us&apiKey={apiKey}").json()
    showNews(response)

elif categoryNum == 7:
    pass

else:
    response = (requests.get(f"http://newsapi.org/v2/top-headlines?country=us&category={categoryDict[categoryNum]}&apiKey={apiKey}"))#.json())
    # rather than .json(), we can also do json.loads(JsonToPython)
    response = json.loads(response.text) # converting string Json to Python disc
    showNews(response)


