/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("2chg320l3jt749c")

  collection.name = "chapters"

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("2chg320l3jt749c")

  collection.name = "chapter"

  return dao.saveCollection(collection)
})
