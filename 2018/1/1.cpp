#include <iostream>
#include <fstream>
#include <list>
#include <iterator>
#include <string>
#include <set>

// class operand {
//   char op;
//   int n;
// public:
//   operand(char *x);
// }
//
// operand::operand(char *x) {
//   op = x[0];
// }

int main(int argc, char *argv[]) {
  if (argc!=2) {
    std::cout << "Invalid argument(s).";
    return 1;
  }

  std::ifstream inputstream(argv[1]);
  if(!inputstream) {
    std::cout << "Cannot open file.";
    return -1;
  }

  else std::cout << "File opened successfully!" << std::endl;

  std::list<std::string> inputarray;
  char line[20];
  // int x = 0;
  while(inputstream) {
    inputstream.getline(line, 20);
    inputarray.push_back(line);
    // std::cout << "Line " << x << ": " << line << std::endl;
    // x++;
  }
  inputarray.pop_back();

  inputstream.close();

  std::list<std::string>::iterator inputiterator = inputarray.begin();

  //std::cout << "You have now reached the print function." << std::endl;

  int freq=0;
  std::set<int> freqarray;
  std::list<int> repeatedfreq;
  std::list<int>::iterator repeatiterator = repeatedfreq.begin();

  try {
    while(inputiterator != inputarray.end()) {
      // std::cout << *inputiterator;
      freq += std::stoi(*inputiterator);
      if(freqarray.find(freq) != freqarray.end()) {
        std::cout << "Repeated frequency: " << freq << std::endl;
        repeatedfreq.push_back(freq);
      }
      else freqarray.insert(freq);
      inputiterator++;
      // std::cout << "Test iterator" << std::endl;
    }
  } catch (std::invalid_argument ia) {
    std::cout << ia.what() << std::endl;
  }

  // while(repeatiterator != repeatedfreq.end()) {
  //   std::cout << "Repeated: " << repeatedfreq;
  // }

  std::cout << "Result: " << freq << std::endl;

  while(repeatedfreq.empty()) {
    if(inputiterator == inputarray.end()) {
      inputiterator = inputarray.begin();
    }
    try {
      while(inputiterator != inputarray.end()) {
        // std::cout << *inputiterator;
        freq += std::stoi(*inputiterator);
        if(freqarray.find(freq) != freqarray.end()) {
          std::cout << "Repeated frequency: " << freq << std::endl;
          return 0;
          repeatedfreq.push_back(freq);
        }
        else freqarray.insert(freq);
        inputiterator++;
        // std::cout << "Test iterator" << std::endl;
      }
    } catch (std::invalid_argument ia) {
      std::cout << ia.what() << std::endl;
    }
  }

  return 0;
}
