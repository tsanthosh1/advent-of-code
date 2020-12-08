import {fileToString, isBetween} from "../util";


export const readPassportDataFromInput = input => {
  let passportData = input.split('\n\n');

  return passportData.map(data => {
    let fields = data.trim().split(/\s+/)

    let readPassport = (agg, field) => {
      let split = field.split(':');
      return {...agg, [split[0]]: split[1]};
    };

    return fields.reduce(readPassport, {})
  })
};

export const readPassportDataFromFile = async () => {
  let fileData = await fileToString('data4');

  return readPassportDataFromInput(fileData);
}

export const getValidPassports = (passports) => {
  const mandatoryFields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

  return passports.filter(passport => {
    return mandatoryFields.reduce((agg, field) => {
      return agg && Boolean(passport[field])
    }, true)
  })
}

export const getValidPassportsWithStrictValidation = (passports) => {
  const validations = {
    byr: (value) => Boolean(value) && value.length === 4 && isBetween(parseInt(value), 1920, 2002),
    iyr: (value) => Boolean(value) && value.length === 4 && isBetween(parseInt(value), 2010, 2020),
    eyr: (value) => Boolean(value) && value.length === 4 && isBetween(parseInt(value), 2020, 2030),
    hgt: (value) => {
      if (!value) return false;
      if (value.endsWith("cm"))
        return !isNaN(value.replace("cm", "")) && isBetween(parseInt(value), 150, 193)
      else if (value.endsWith("in"))
        return !isNaN(value.replace("in", "")) && isBetween(parseInt(value), 59, 76)
      else return false
    },
    hcl: (value) => Boolean(value) && Boolean(value.match(/^#[0-9a-f]{6}$/)),
    ecl: (value) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].includes(value),
    pid: (value) => !isNaN(value) && Boolean(value.match(/^[0-9]{9}$/)),
  }

  return passports.filter(passport => {
    return Object.keys(validations).reduce((agg, field) =>
        agg && validations[field](passport[field]), true)
  })
}

(async () => {
  let passports = await readPassportDataFromFile();
  console.log(getValidPassports(passports).length)
  console.log(getValidPassportsWithStrictValidation(passports).length)
})()
