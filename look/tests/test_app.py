import io

import falcon
from falcon import testing
import msgpack

import pytest
from unittest.mock import mock_open, call, MagicMock

import look.app
import look.images


@pytest.fixture
def mock_store():
    return MagicMock()


@pytest.fixture
def client(mock_store):
    app = look.app.create_app(mock_store)
    return testing.TestClient(app)


def test_list_images(client):
    images = [
        {
            'href': 'http://i0.kym-cdn.com/photos/images/facebook/001/365/839/24d.png'
        }
    ]
    resp = client.simulate_get('/images')
    results = msgpack.unpackb(resp.content, encoding='utf-8')

    assert results == images
    assert resp.status == falcon.HTTP_OK


def test_post_image(client, mock_store):
    file_name = 'fake-image-name.xyz'

    mock_store.save.return_value = file_name
    image_content_type = 'image/xyz'
    
    resp = client.simulate_post(
        '/images',
        body=b'fake-image-bytes',
        headers={'content-type': image_content_type}
    )

    assert resp.status == falcon.HTTP_CREATED
    assert resp.headers['location'] == '/images/{}'.format(file_name)

    save_call = mock_store.save.call_args # (positional args, ...)
    assert isinstance(save_call[0][0], falcon.request_helpers.BoundedStream)
    assert save_call[0][1] == image_content_type


def test_save_image():
    fake_uuid = '123e4567-e89b-12d3-a456-426655440000'
    def mock_uuidgen():
        return fake_uuid

    mock_file_open = mock_open()
    image_store = look.images.ImageStore('fake_storage_path', uuidgen=mock_uuidgen, fopen=mock_file_open)

    fake_image_bytes = b'fake-image-bytes'
    fake_req_stream = io.BytesIO(fake_image_bytes)

    assert image_store.save(fake_req_stream, 'image/png') == fake_uuid + '.png'
    assert call().write(fake_image_bytes) in mock_file_open.mock_calls
