apiKey = ''
with open("MyApiForNewsApi.key","r") as filePointer:
    apiKey = filePointer.read()


import requests
import argparse


def showNews(response):
    articles = response['articles']
    num = 1
    for i in range(len(articles)):
        if (articles[i]['content']) == None:
            continue

        print(f"\n article {i+1} {articles[i]['description']}")
        num+=1

# From command line

parser = argparse.ArgumentParser()
parser.add_argument('--news',default=None,action="store_true", help='URL of the news') 
parser.add_argument('category', nargs="?", default=None, help="type category") # nargs = "?" means it is optional
args = parser.parse_args()

if args.news == None:
    pass

elif args.category == None:
    try:
        response = requests.get(f"https://newsapi.org/v2/top-headlines?country=us&apiKey={apiKey}").json()
    except Exception as e:
        print("error occured", e)
    showNews(response)

else:
    response = requests.get(f"https://newsapi.org/v2/top-headlines?country=us&category={args.category}&apiKey={apiKey}")
    if 'Content-Length' not in response.headers: # for this site, it doesn't send content-length if site is matched
        showNews(response.json())

    else:
        print("Error Occured, the category send was not valid")
        


