/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("gib2oh1ms3uymes")

  // update
  collection.schema.addField(new SchemaField({
    "system": false,
    "id": "nct3xh4f",
    "name": "translated_language",
    "type": "text",
    "required": false,
    "unique": false,
    "options": {
      "min": null,
      "max": 2,
      "pattern": ""
    }
  }))

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("gib2oh1ms3uymes")

  // update
  collection.schema.addField(new SchemaField({
    "system": false,
    "id": "nct3xh4f",
    "name": "translated_language",
    "type": "text",
    "required": false,
    "unique": false,
    "options": {
      "min": null,
      "max": null,
      "pattern": ""
    }
  }))

  return dao.saveCollection(collection)
})
