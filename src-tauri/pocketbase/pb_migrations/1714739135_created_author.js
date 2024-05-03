/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const collection = new Collection({
    "id": "km7esrzuj1hpgw8",
    "created": "2024-05-03 12:25:35.250Z",
    "updated": "2024-05-03 12:25:35.250Z",
    "name": "author",
    "type": "base",
    "system": false,
    "schema": [
      {
        "system": false,
        "id": "gadjogu9",
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
        "id": "2aqn2719",
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
  const collection = dao.findCollectionByNameOrId("km7esrzuj1hpgw8");

  return dao.deleteCollection(collection);
})
