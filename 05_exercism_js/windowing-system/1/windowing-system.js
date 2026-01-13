// @ts-check

/**
 * Implement the classes etc. that are needed to solve the
 * exercise in this file. Do not forget to export the entities
 * you defined so they are available for the tests.
 */
export class Size {
  constructor( a = 80 , b = 60 ) {
    this.width = a 
    this.height = b
  }
  resize( a , b ) {
    this.width = a 
    this.height = b
  }
}

export class Position {
  constructor( a = 0 , b = 0 ) {
    this.x = a 
    this.y = b
  }
  move( a , b ) {
    this.x = a 
    this.y = b
  }
}
export class ProgramWindow {
  constructor() {
    this.screenSize = new Size(800, 600);
    this.size = new Size();
    this.position = new Position();
  }

  resize(newSize) {
    const minWidth = 1;
    const minHeight = 1;

    const maxWidth = this.screenSize.width - this.position.x;
    const maxHeight = this.screenSize.height - this.position.y;

    const width = Math.min(
      Math.max(newSize.width, minWidth),
      maxWidth
    );

    const height = Math.min(
      Math.max(newSize.height, minHeight),
      maxHeight
    );

    this.size.width = width;
    this.size.height = height ;
  }
  move(newPosition) {
    const maxX = this.screenSize.width - this.size.width;
    const maxY = this.screenSize.height - this.size.height;
  
    this.position.x = Math.min(Math.max(newPosition.x, 0), maxX);
    this.position.y = Math.min(Math.max(newPosition.y, 0), maxY);
  }

}
export function changeWindow(programWindow) {
  programWindow.resize(new Size(400, 300));
  programWindow.move(new Position(100, 150));
  return programWindow;
}

