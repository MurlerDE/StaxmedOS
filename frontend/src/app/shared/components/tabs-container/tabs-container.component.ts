import { Component, contentChildren, AfterContentInit } from '@angular/core';
import { NgClass } from '@angular/common';
import { find } from 'rxjs';
import { TabComponent } from '../tab/tab.component';

@Component({
  selector: 'app-tabs-container',
  imports: [NgClass],
  templateUrl: './tabs-container.component.html',
  styleUrls: ['./tabs-container.component.css'],
})
export class TabsContainerComponent implements AfterContentInit {
  tabs = contentChildren(TabComponent);

  ngAfterContentInit(): void {
    const activeTab = this.tabs().find((tab) => tab.active());
    if (!activeTab) {
      this.selectTab(this.tabs()[0]);
    }
  }

  selectTab(tab: TabComponent) {
    this.tabs().forEach((tab) => tab.active.set(false));
    tab.active.set(true);
    return false;
  }
}
