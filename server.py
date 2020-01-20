import tornado.ioloop
import tornado.web
import tornado.websocket

import logging

logging.basicConfig( level=logging.DEBUG )

class EchoWebSocket(tornado.websocket.WebSocketHandler):

    def check_origin( self, origin ):
        return True 

    def open(self):
        print("WebSocket opened", self.request.remote_ip )
        self.write_message(u"Hi I'm server")

    def on_message(self, message):
        print( f"Got message: {message}")

    def on_close(self):
        print("WebSocket closed")

def make_app():
    return tornado.web.Application([
        (r"/ws", EchoWebSocket),
    ], debug=True )

if __name__ == "__main__":
    app = make_app()
    app.listen(5555)
    tornado.ioloop.IOLoop.current().start()