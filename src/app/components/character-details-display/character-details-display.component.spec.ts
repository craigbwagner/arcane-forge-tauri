import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CharacterDetailsDisplayComponent } from './character-details-display.component';

describe('CharacterDetailsDisplayComponent', () => {
  let component: CharacterDetailsDisplayComponent;
  let fixture: ComponentFixture<CharacterDetailsDisplayComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CharacterDetailsDisplayComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(CharacterDetailsDisplayComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
