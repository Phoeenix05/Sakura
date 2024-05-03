/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const collection = new Collection({
    "id": "apgl6d6wwuektb0",
    "created": "2024-05-03 12:27:13.230Z",
    "updated": "2024-05-03 12:27:13.230Z",
    "name": "publisher",
    "type": "base",
    "system": false,
    "schema": [
      {
        "system": false,
        "id": "wc5azqlc",
        "name": "name",
        "type": "text",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "min": null,
          "max": null,
          "pattern": ""
        }
      },
      {
        "system": false,
        "id": "uas0vp8m",
        "name": "page",
        "type": "url",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "exceptDomains": null,
          "onlyDomains": null
        }
      }
    ],
    "indexes": [],
    "listRule": null,
    "viewRule": null,
    "createRule": null,
    "updateRule": null,
    "deleteRule": null,
    "options": {}
  });

  return Dao(db).saveCollection(collection);
}, (db) => {
  const dao = new Dao(db);
  const collection = dao.findCollectionByNameOrId("apgl6d6wwuektb0");

  return dao.deleteCollection(collection);
})
