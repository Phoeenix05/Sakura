/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("gib2oh1ms3uymes")

  // remove
  collection.schema.removeField("ibq4xe8y")

  // add
  collection.schema.addField(new SchemaField({
    "system": false,
    "id": "rdcwg5ib",
    "name": "publish_at",
    "type": "date",
    "required": true,
    "unique": false,
    "options": {
      "min": "",
      "max": ""
    }
  }))

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("gib2oh1ms3uymes")

  // add
  collection.schema.addField(new SchemaField({
    "system": false,
    "id": "ibq4xe8y",
    "name": "publish_at",
    "type": "text",
    "required": true,
    "unique": false,
    "options": {
      "min": null,
      "max": null,
      "pattern": "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(\\.[0-9]+)?([Zz]|([\\+-])([01]\\d|2[0-3]):?([0-5]\\d)?)?"
    }
  }))

  // remove
  collection.schema.removeField("rdcwg5ib")

  return dao.saveCollection(collection)
})
