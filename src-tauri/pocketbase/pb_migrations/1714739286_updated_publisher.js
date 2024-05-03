/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("apgl6d6wwuektb0")

  collection.name = "publishers"

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("apgl6d6wwuektb0")

  collection.name = "publisher"

  return dao.saveCollection(collection)
})
