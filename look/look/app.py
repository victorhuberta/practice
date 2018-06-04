import falcon

from .images import ImagesResource, ImageStore


def create_app(image_store):
    app = application = falcon.API()
    images = ImagesResource(image_store)
    app.add_route('/images', images)
    return app
    
    
def get_app():
    image_store = ImageStore('.')
    return create_app(image_store)
