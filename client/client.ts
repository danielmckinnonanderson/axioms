
export type ClientMessage = {
  type: ClientMessageType;
  payload: ClientMessagePayload;
};

export enum ClientMessageType {
  ReadyStatusChanged = 0x00,
};

export type ClientMessagePayload =
  | ReadyStatusChangedPayload;

export type ReadyStatusChangedPayload = boolean;

export class GameClientConnection {
  private conn: WebSocket | undefined = undefined;

  public constructor(connUrl: string, subprotocol: string) {
    this.conn = new WebSocket(connUrl, subprotocol);
  }

  private static serializeHeader(msgType: ClientMessageType, buf: ArrayBuffer) {

  }

  public send(message: ClientMessage): any | Error {

  }
}

