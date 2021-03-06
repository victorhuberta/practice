import os
import re
import mimetypes
import uuid
import io

import falcon
import msgpack


ALLOWED_IMAGE_TYPES = (
    'image/gif',
    'image/jpeg',
    'image/png',
)

def validate_image_type(req, resp, resource, params):
    if req.content_type not in ALLOWED_IMAGE_TYPES:
        msg = 'Image type not allowed. Guess again!'
        raise falcon.HTTPBadRequest('Bad request', msg)


class Collection(object):

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


    @falcon.before(validate_image_type)
    def on_post(self, req, resp):
        name = self._image_store.save(req.stream, req.content_type)
        resp.status = falcon.HTTP_201
        resp.location = '/images/' + name


class Item(object):

    def __init__(self, image_store):
        self._image_store = image_store


    def on_get(self, req, resp, name):
        resp.content_type = mimetypes.guess_type(name)[0]
        try:
            resp.stream, resp.stream_len = self._image_store.open(name)
        except IOError:
            raise falcon.HTTPNotFound()


class Store(object):

    _CHUNK_SIZE_BYTES = 4096
    _IMAGE_NAME_PATTERN = re.compile(
        '[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\.[a-z]{2,4}$'
    )
    
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


    def open(self, name):
        if not self._IMAGE_NAME_PATTERN.match(name):
            raise IOError('File not found')

        image_path = os.path.join(self._storage_path, name)
        stream = self._fopen(image_path, 'rb')
        stream_len = os.path.getsize(image_path)
        return stream, stream_len
