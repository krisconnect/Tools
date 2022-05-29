from random import randrange
from typing import Optional

from fastapi import FastAPI, Response, status, HTTPException
from fastapi.params import Body
from pydantic import BaseModel

app = FastAPI()

class Post(BaseModel):
    character_name: str
    lifeform: str
    class_or_type: str
    story: Optional[str] = None
    strength: int
    dexterity: int
    constitution: int
    intelligence: int
    wisdom: int
    cha: int
    basic: int
    weapons_and_tools: int
    guns: int
    energy_or_magic: int
    ultimate: int
    hp: int
    defense: int
    loot: str
    ability1: str
    ability2: str
    ability3: str
    ability4: str
    ability5: str
    power1: str
    power2: str
    power3: str
    auguments: str

local_posts = []

def find_post(id):
    for p in local_posts:
        if p["id"] == id:
            return p

def find_index_post(id):
    for i,p in enumerate(local_posts):
        if p['id'] == id:
            return i

@app.get("/")
def root():
    return {"message": "You get a char, and you get a char..."}
    

@app.post("/posts", status_code=status.HTTP_201_CREATED)
def create_post(post: Post):
    print(post.dict())
    post_dict = post.dict()
    post_dict['id'] = randrange(0, 1000000)
    local_posts.append(post_dict)
    return{"data": post_dict}


@app.get("/posts")
def get_posts():
    return{"data": local_posts}

# Remember that the order matters. /latest could've caused and issue if it was below {id}
@app.get("/posts/latest")
def getLatestPost():
    latestPost = local_posts[len(local_posts)-1]
    return{"latest": latestPost}


@app.get("/posts/{id}")
def get_post(id: int, response: Response):
    post = find_post(id)
    if not post:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND,
                            detail=f"The post with id: {id} was not found!")
        #response.status_code = status.HTTP_404_NOT_FOUND
        #return{"message": f"The post with id: {id} was not found!"}
    return {"post_detail": post}


@app.delete("/posts/{id}", status_code=status.HTTP_204_NO_CONTENT)
def delete_post(id):
    #1, find the id
    #2, local_posts.pop(index)
    index = find_index_post(id)
    
    if index == None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail=f"Post with id: {id} does not exist.")
    local_posts.pop(index)
    return Response(status_code=status.HTTP_204_NO_CONTENT)

@app.put("/posts/{id}")
def update_post(id: int, post: Post):
    index = find_index_post(id)
    
    if index == None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail=f"Post with id: {id} does not exist.")
    post_dict = post.dict()
    post_dict['id'] = id
    local_posts[index] = post_dict
    return{"message": post_dict}
