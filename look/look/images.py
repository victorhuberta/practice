import os
import mimetypes
import uuid
import io

import falcon
import msgpack


class ImagesResource(object):

    _CHUNK_SIZE_BYTES = 4096

    def __init__(self, storage_path):
        self._storage_path = storage_path


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
        ext = mimetypes.guess_extension(req.content_type)
        name = '{uuid}{ext}'.format(uuid=uuid.uuid4(), ext=ext)
        image_path = os.path.join(self._storage_path, name)

        with io.open(image_path, 'wb') as image_file:
            while True:
                chunk = req.stream.read(self._CHUNK_SIZE_BYTES)
                if not chunk:
                    break
                image_file.write(chunk)

        resp.status = falcon.HTTP_201
        resp.location = '/images/' + name
