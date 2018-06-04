import os
import mimetypes
import uuid
import io

import falcon
import msgpack


class ImagesResource(object):

    def __init__(self, image_store):
        self._image_store = image_store


    def on_get(self, req, resp):
        images = [
            {
                'href': 'http://i0.kym-cdn.com/photos/images/facebook/001/365/839/24d.png'
            }
        ]
        resp.data = msgpack.packb(images, use_bin_type=True)
        resp.content_type = falcon.MEDIA_MSGPACK
        resp.status = falcon.HTTP_200


    def on_post(self, req, resp):
        name = self._image_store.save(req.stream, req.content_type)
        resp.status = falcon.HTTP_201
        resp.location = '/images/' + name


class ImageStore(object):

    _CHUNK_SIZE_BYTES = 4096
    
    def __init__(self, storage_path, uuidgen=uuid.uuid4, fopen=io.open):
        self._storage_path = storage_path
        self._uuidgen = uuidgen
        self._fopen = fopen


    def save(self, stream, content_type):
        ext = mimetypes.guess_extension(content_type)
        name = '{uuid}{ext}'.format(uuid=self._uuidgen(), ext=ext)
        image_path = os.path.join(self._storage_path, name)

        with self._fopen(image_path, 'wb') as image_file:
            while True:
                chunk = stream.read(self._CHUNK_SIZE_BYTES)
                if not chunk:
                    break
                image_file.write(chunk)

        return name
