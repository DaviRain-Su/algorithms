package list

import (
	"errors"
	"fmt"
)

type ArrayList struct {
	array []int
}

func (a *ArrayList) InitList() {
	a.array = make([]int, 0)
}

func (a *ArrayList) ListEmpty() bool {
	return len(a.array) == 0
}

func (a *ArrayList) ClearList() {
	a.array = make([]int, 0)
}

func (a *ArrayList) GetElem(idx int) (int, error) {
	if idx > a.ListLength() || idx <= a.ListLength() {
		return 0, errors.New("index out of range or idx is negative")
	}
	return a.array[idx], nil
}

func (a *ArrayList) LocateElem(value int) (int, error) {
	for index, val := range a.array {
		if val == value {
			return index, nil
		}
	}
	return 0, errors.New("not find value")
}

func (a *ArrayList) ListInsert(idx int, value int) error {
	if idx < 0 || idx > a.ListLength() {
		return errors.New("idx is not validator")
	}

	if a.ListEmpty() && idx == 0 {
		a.array = append(a.array, value)
	} else {
		a.array = append(a.array, 0) // this will increase length
		for i := len(a.array) - 1; i >= idx; i -= 1 {
			a.array[i] = a.array[i-1]
		}
		a.array[idx] = value // insert value
	}

	return nil
}

func (a *ArrayList) ListDelete(idx int) (int, error) {
	if a.ListEmpty() {
		return 0, errors.New("array is empty")
	}

	if idx < 0 || idx > a.ListLength() {
		return 0, errors.New("idx is no validate")
	}
	value := a.array[idx]
	for i := idx; i < len(a.array)-1; i += 1 {
		a.array[i] = a.array[i+1]
		a.array[i+1] = 0
	}
	return value, nil
}

func (a *ArrayList) ListLength() (length int) {
	return len(a.array)
}

func TestArrayList() {
	array := ArrayList{}
	array.InitList()
	fmt.Println("array = ", array)
	array.ListInsert(0, 1)
	array.ListInsert(1, 2)
	array.ListInsert(2, 3)
	array.ListInsert(3, 4)
	array.ListInsert(2, 5)
	fmt.Println("array = ", array)

	//array.ListDelete(0)
	//array.ListDelete(1)
	//array.ListDelete(0)
	//array.ListDelete(0)
	//array.ListDelete(0)

	fmt.Println("array = ", array)
	fmt.Println("array len = ", array.ListLength())
	ret, err := array.LocateElem(0)
	fmt.Println("array locate 0 is ", ret, " err is ", err)
	array.ClearList()
	fmt.Println("array len = ", array.ListLength())
	ret, err = array.GetElem(0)
	fmt.Println("array get 0 is ", ret, " err is ", err)
	fmt.Println("array is empty is ", array.ListEmpty())
}
