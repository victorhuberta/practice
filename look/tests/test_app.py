import falcon
from falcon import testing

import pytest

from unittest.mock import mock_open, call

import msgpack

from look.app import app


@pytest.fixture
def client():
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


def test_posted_images_get_saved(client, monkeypatch):
    mock_file_open = mock_open()
    monkeypatch.setattr('io.open', mock_file_open)

    fake_uuid = '123e4567-e89b-12d3-a456-426655440000'
    monkeypatch.setattr('uuid.uuid4', lambda: fake_uuid)

    fake_image_bytes = b'fake-image-bytes'
    resp = client.simulate_post(
        '/images',
        body=fake_image_bytes,
        headers={'content-type': 'image/png'}
    )

    assert resp.status == falcon.HTTP_CREATED
    assert call().write(fake_image_bytes) in mock_file_open.mock_calls
    assert resp.headers['location'] == '/images/{}.png'.format(fake_uuid)
