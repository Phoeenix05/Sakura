/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("km7esrzuj1hpgw8")

  collection.name = "authors"

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("km7esrzuj1hpgw8")

  collection.name = "author"

  return dao.saveCollection(collection)
})
