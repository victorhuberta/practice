import os
import falcon

import look.images as images


def create_app(image_store):
    app = application = falcon.API()
    app.add_route('/images', images.Collection(image_store))
    app.add_route('/images/{name}', images.Item(image_store))
    return app
    
    
def get_app():
    storage_path = os.environ.get('LOOK_STORAGE_PATH', '.')
    image_store = images.Store(storage_path)
    return create_app(image_store)
