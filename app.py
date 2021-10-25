from PyQt5.QtGui import * 
from PyQt5.QtWidgets import * 
  
  
app = QApplication([])
app.setQuitOnLastWindowClosed(False)
  
# Adding an icon
icon = QIcon("audio-headphones.svg")
  
# Adding item on the menu bar
tray = QSystemTrayIcon()
tray.setIcon(icon)
tray.setVisible(True)
  
# Creating the options
menu = QMenu()
option1 = QAction("HyperX Cloud Flight")
option1.setDisabled(True)

option2 = QAction("GFG")
menu.addAction(option1)
menu.addAction(option2)
  
# To quit the app
quit = QAction("Quit")
quit.triggered.connect(app.quit)
menu.addAction(quit)
  
# Adding options to the System Tray
tray.setContextMenu(menu)
  
app.exec_()