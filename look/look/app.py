import falcon

from .images import ImagesResource


app = application = falcon.API()

images = ImagesResource(storage_path='.')
app.add_route('/images', images)
