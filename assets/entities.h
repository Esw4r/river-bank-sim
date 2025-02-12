#include <string>

#import "names.h"

typedef unsigned char uint8;
using namespace std;

class Animal {
private:
  string name[2];
  uint8 health;
  uint8 age;
  uint8 expectancy;
  bool gender;
  uint8 hunger;
  uint8 thirst;
  uint8 speed;
  uint8 vision;
  uint8 stamina;
  uint8 strength;
  uint8 direction[2];
  uint8 eyes[2];
  uint8 ears[2];
  uint8 limbs[4];
  uint8 head;
  uint8 torso;
  uint8 genitals;
  uint8 fov;
  uint8 posn[2];
  uint8 fear;
  uint8 herdTend;

public:
  Animal() {}

  string *getName() { return name; }
  void setName(string firstName, string lastName) {
    name[0] = firstName;
    name[1] = lastName;
  }

  uint8 getHealth() { return health; }
  void setHealth(uint8 x) { health = x; }

  uint8 getAge() { return age; }
  void setAge(uint8 x) { age = x; }

  uint8 getExp() { return expectancy; }
  void setExp(uint8 x) { expectancy = x; }

  bool getGender() { return gender; }
  void setGender(bool x) { gender = x; }

  uint8 getHunger() { return hunger; }
  void setHunger(uint8 x) { hunger = x; }

  uint8 getThirst() { return thirst; }
  void setThirst(uint8 x) { thirst = x; }

  uint8 getSpeed() { return speed; }
  void setSpeed(uint8 x) { speed = x; }

  uint8 getVision() { return vision; }
  void setVision(uint8 x) { vision = x; }

  uint8 *getEars() { return ears; }
  void setEars(uint8 x, uint8 y) {
    ears[0] = x;
    ears[1] = y;
  }

  uint8 *getEyes() { return eyes; }
  void setEyes(uint8 x, uint8 y) {
    eyes[0] = x;
    eyes[1] = y;
  }

  uint8 *getDir() { return direction; }
  void setDirection(uint8 x, uint8 y) {
    direction[0] = x;
    direction[1] = y;
  }

  uint8 *getLimbs() { return limbs; }
  void setLimbs(uint8 w, uint8 x, uint8 y, uint8 z) {
    limbs[0] = w;
    limbs[1] = x;
    limbs[2] = y;
    limbs[3] = z;
  }

  uint8 getHead() { return head; }
  void setHead(uint8 x) { head = x; }

  uint8 getTorso() { return torso; }
  void setTorso(uint8 x) { torso = x; }

  uint8 getGenitals() { return genitals; }
  void setGenitals(uint8 x) { genitals = x; }

  uint8 getFov() { return fov; }
  void setFov(uint8 x) { fov = x; }
};

class Lion : public Animal {
private:
public:
  void search() {}

  Lion reproduce(Lion x, Lion y) {
    Lion z;

    z.setGender(rand() %  2);

    string firstName = (z.getGender()) ? mNames[rand() % 100] : fNames[rand() % 100];
    string lastName = (x.getGender()) ? x.getName()[1] : y.getName()[1];

    z.setName(firstName, lastName);

    

    return z;
  }
};

class Bison : public Animal {
private:
public:
  void search() {}
};